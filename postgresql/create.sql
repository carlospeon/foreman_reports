
CREATE DATABASE foreman_reports;
CREATE USER foreman_reports password '${password}';

ALTER ROLE "foreman_reports" WITH LOGIN;
GRANT CONNECT on DATABASE foreman_reports to foreman_reports;

CREATE EXTENSION postgres_fdw;

CREATE SERVER ${satellite}
    FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (host '${satellite}', dbname 'foreman', fetch_size '10000' );
alter server ${satellite} options (add use_remote_estimate 'on');


CREATE USER MAPPING FOR foreman_reports
    SERVER ${satellite}
    OPTIONS (USER 'foreman_reports', password '${password_satellite}');

CREATE SERVER ${satellite_aux}
    FOREIGN DATA WRAPPER postgres_fdw
    OPTIONS (host '${satellite_aux}', dbname 'foreman', fetch_size '10000' );
alter server ${satellite_aux} options (add use_remote_estimate 'on');
CREATE USER MAPPING FOR foreman_reports
    SERVER ${satellite_aux}
    OPTIONS (USER 'foreman_reports', password '${password_satellite_aux}');

CREATE schema digital;
IMPORT FOREIGN SCHEMA public LIMIT TO (
hosts,
fact_names,
fact_values,
katello_content_facet_errata,
katello_content_facet_repositories,
katello_content_facets,
katello_content_view_environment_content_facets,
katello_content_view_environments,
katello_content_view_histories,
katello_content_view_versions,
katello_content_views,
katello_environments,
katello_errata,
katello_erratum_cves,
katello_repositories,
katello_repository_errata,
katello_systems,
logs,
nics,
reports,
sources,
subnets,
taxonomies
)
FROM SERVER ${satellite} INTO digital;
GRANT usage on schema digital to foreman_reports;
GRANT select on digital.reports to foreman_reports;
GRANT select on digital.logs to foreman_reports;
GRANT select on digital.sources to foreman_reports;


CREATE schema sscc;
IMPORT FOREIGN SCHEMA public LIMIT TO (
hosts,
fact_names,
fact_values,
katello_content_facet_errata,
katello_content_facet_repositories,
katello_content_facets,
katello_content_view_histories,
katello_content_view_versions,
katello_environments,
katello_errata,
katello_erratum_cves,
katello_repository_errata,
katello_systems,
logs,
nics,
reports,
sources,
subnets,
taxonomies
)
FROM SERVER ${satellite_aux} INTO sscc;

GRANT usage on schema sscc to foreman_reports;
GRANT select on sscc.reports to foreman_reports;
GRANT select on sscc.logs to foreman_reports;
GRANT select on sscc.sources to foreman_reports;

CREATE TABLE materialized_views (
  name text primary key,
  refreshed_at TIMESTAMP WITH TIME ZONE
);
GRANT select,INSERT,update on materialized_views to foreman_reports;

DROP MATERIALIZED VIEW digital_erratas;
CREATE MATERIALIZED VIEW IF NOT EXISTS digital_erratas
AS
select
    ke.errata_id,
    ke.title,
    ke.errata_type,
    ke.severity,
    ke.issued,
    ke.reboot_suggested,
    cves.cve_list
from digital.katello_errata ke
    left join (
        select erratum_id,  string_agg(cve_id, ',') as cve_list
        from digital.katello_erratum_cves
        group by erratum_id
     ) cves on
        ke.id = cves.erratum_id
    where
        ke.id in (
          select distinct erratum_id from digital.katello_content_facet_errata 
          where
            content_facet_id in (
              select id from digital.katello_content_facets where installable_security_errata_count > 0
            )
        ) and
        ke.issued > current_date - interval '10 years' and
        ke.errata_type = 'security'
with data;

CREATE UNIQUE INDEX digital_erratas_id on digital_erratas(errata_id);
CREATE INDEX digital_erratas_issued on digital_erratas(issued);
GRANT select on digital_erratas to foreman_reports;
INSERT INTO materialized_views (name, refreshed_at) VALUES ('digital_erratas', CURRENT_TIMESTAMP);

CREATE TABLE content_views (
  bu text,
  id integer,
  filter text,
  compliant_update_date date,
  ccv_id integer,
  primary key(bu, ccv_id)
);
GRANT select on content_views to foreman_reports;
insert into content_views (bu, id, filter, ccv_id) values ('digital', 80, '%', 16);
insert into content_views (bu, id, filter, ccv_id) values ('digital', 86, '%', 85); -- layer 0
insert into content_views (bu, id, filter, ccv_id) values ('sscc', 3, '%', 17);
insert into content_views (bu, id, filter, ccv_id) values ('sscc', 62, '%', 61); --layer 0



DROP MATERIALIZED VIEW content_view_releases;
CREATE MATERIALIZED VIEW IF NOT EXISTS content_view_releases
AS
with cv_digital (bu, id, filter) as (
  select bu, id, filter from content_views where bu = 'digital'
),
cv_sscc (bu, id, filter) as (
  select bu, id, filter from content_views where bu = 'sscc'
)
select
  v.id,
  v.content_view_id,
  cv_digital.bu,
  v.created_at::date, 
  h.notes
from 
  digital.katello_content_view_versions v 
    join digital.katello_content_view_histories h on 
      v.id = h.katello_content_view_version_id and 
      h.action=1
    join cv_digital on 
      content_view_id = cv_digital.id and
      h.notes like cv_digital.filter
union all
select
  v.id,
  v.content_view_id,
  cv_sscc.bu,
  v.created_at::date, 
  h.notes
from 
  sscc.katello_content_view_versions v 
    join sscc.katello_content_view_histories h on 
      v.id = h.katello_content_view_version_id and 
      h.action=1
    join cv_sscc on 
      content_view_id = cv_sscc.id and
      h.notes like cv_sscc.filter
with data;
CREATE UNIQUE INDEX content_view_releases_id on content_view_releases(bu, id);
GRANT select on content_view_releases to foreman_reports;
INSERT INTO materialized_views (name, refreshed_at) VALUES ('content_view_releases', CURRENT_TIMESTAMP);

CREATE OR REPLACE PROCEDURE refresh_materialized_view(in_name text, ttl interval)
SECURITY DEFINER
LANGUAGE plpgsql
AS $$
DECLARE
  time_old interval := 0;
  r record;
  compliant_update_ts timestamp;
BEGIN
  select INTO time_old CURRENT_TIMESTAMP - refreshed_at 
    from materialized_views mv
    where mv.name = in_name;
  if time_old > ttl then
    select INTO time_old CURRENT_TIMESTAMP - refreshed_at 
      from materialized_views mv
      where mv.name = in_name
      for update SKIP LOCKED;
    if time_old is not null and time_old > ttl then
      RAISE NOTICE 'Refreshing %', in_name;
      execute 'REFRESH MATERIALIZED VIEW CONCURRENTLY ' || in_name || ' WITH DATA;';

      if in_name = 'content_view_releases' then
        for r in select bu, id, filter from content_views
        loop
          update content_views set compliant_update_date = q.created_at 
          from (
            select created_at
            from content_view_releases 
            where bu = r.bu and content_view_id = r.id and notes like r.filter
            order by 1 desc limit 1 offset 1
          ) as q
          where bu = r.bu and id = r.id;
        end loop;
      end if;

      update materialized_views set refreshed_at = CURRENT_TIMESTAMP
        where name = in_name;
      RAISE NOTICE 'done';
    end if;
  end if;
END;
$$;
GRANT all on procedure refresh_materialized_view to foreman_reports;


CREATE TABLE history_report_queries (
  id serial primary key,
  sql text
);
GRANT select,insert,update on history_report_queries to foreman_reports;

INSERT INTO history_report_queries (sql) VALUES (
  'select ''hosts_groupby_osmajor'', now(), json_agg(r) as report from (
    select 
      case when facts->>''os_version'' is null then
        ''-N/A-''
      else
        ''RHEL '' || substring(facts->>''os_version'', 0, position(''.'' in facts->>''os_version''))
      end as key,
      count(id) as value
    from full_report
    group by key
    order by value desc) as r'
);
INSERT INTO history_report_queries (sql) VALUES (
  'select ''hosts_groupby_hardware'', now(), json_agg(r) as report from (
    select 
      case when facts->>''product_name'' is null then
        ''-N/A-''
      else
        facts->>''product_name''
      end as key,
      count(id) as value
    from full_report
    group by key
    order by value desc) as r'
);
INSERT INTO history_report_queries (sql) VALUES (
  'select ''hosts_groupby_domain'', now(), json_agg(r) as report from (
    select 
      substring(hostname, position(''.'' in hostname) + 1) as key,
      count(id) as value
    from full_report
    group by key
    order by value desc) as r'
);
INSERT INTO history_report_queries (sql) VALUES (
  'select ''cpus_groupby_fact'', now(), json_agg(r) as report from (
  select t.key, t.value from ( 
    select 
      sum( case when facts->>''db2_version'' is not null and facts->>''db2_version'' != '''' and facts->>''db2_version'' != ''None'' and facts->>''db2_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as db2_version,
      sum( case when facts->>''kong_version'' is not null and facts->>''kong_version'' != '''' and facts->>''kong_version'' != ''None'' and facts->>''kong_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as kong_version,
      sum( case when facts->>''wmq_version'' is not null and facts->>''wmq_version'' != '''' and facts->>''wmq_version'' != ''None'' and facts->>''wmq_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as wmq_version,
      sum( case when facts->>''was_version'' is not null and facts->>''was_version'' != '''' and facts->>''was_version'' != ''None'' and facts->>''was_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as was_version,
      sum( case when facts->>''ihs_version'' is not null and facts->>''ihs_version'' != '''' and facts->>''ihs_version'' != ''None'' and facts->>''ihs_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as ihs_version,
      sum( case when facts->>''wxsserver_version'' is not null and facts->>''wxsserver_version'' != '''' and facts->>''wxsserver_version'' != ''None'' and facts->>''wxsserver_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as wxsserver_version,
      sum( case when facts->>''couchbase_version'' is not null and facts->>''couchbase_version'' != '''' and facts->>''couchbase_version'' != ''None'' and facts->>''couchbase_version'' not like ''% not installed'' then 
        cast(facts->>''online_cpu'' as bigint) else 0 end) as couchbase_version
    from full_report 
  ) f 
  cross join lateral (
    VALUES 
      (''db2_version'', f.db2_version),
      (''kong_version'', f.kong_version),
      (''wmq_version'', f.wmq_version),
      (''was_version'', f.was_version),
      (''ihs_version'', f.ihs_version),
      (''wxsserver_version'', f.wxsserver_version),
      (''couchbase_version'', f.couchbase_version)
    ) as t(key, value)
) r'
);


CREATE TABLE history_reports (
  name text,
  gathered_at TIMESTAMP WITH TIME ZONE,
  report json not null,
  primary key(name, gathered_at)
);
GRANT select,insert,update on history_reports to foreman_reports;
CREATE TYPE history_report_t AS (key TEXT, value INT);

CREATE OR REPLACE PROCEDURE gather_history_reports(ttl interval)
-- SECURITY DEFINER
LANGUAGE plpgsql
AS $$
DECLARE
  r record;
BEGIN
  call refresh_materialized_view('full_report', ttl);
  for r in select sql from history_report_queries
  loop
    execute 'INSERT INTO history_reports (name, gathered_at, report) ' || r.sql;
  end loop;
END;
$$;

CREATE TABLE os (
 major text primary key,
 eol TIMESTAMP WITH TIME ZONE
);
CREATE UNIQUE INDEX os_eol on os(eol);
GRANT select on os to foreman_reports;
INSERT INTO os (major, eol) VALUES ('6', '2020-11-30');
INSERT INTO os (major, eol) VALUES ('7', '2024-06-30');
INSERT INTO os (major, eol) VALUES ('8', '2029-05-31');
INSERT INTO os (major, eol) VALUES ('9', '2032-05-31');


CREATE or replace view supported_os as select major from os where eol > now();
GRANT select on supported_os to foreman_reports;

DROP TABLE facts;
CREATE TABLE facts (
 name text primary key,
 short_name text,
 description text,
 visible bool,
 aggregable bool
 online_cpu bool,
);
GRANT select on facts to foreman_reports;
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('dmi::system::product_name', 'product_name', 'Hardware', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('dmi::system::uuid', 'uuid', 'UUID', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('dmi::system::serial_number', 'serial_number', 'Serial Number', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('virt::is_guest', 'virtual', 'Virtual', false, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('distribution::version', 'os_version', 'OS', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('uname::release', 'kernel_release', 'Kernel release', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_datetime', 'facts_datetime', 'Facts date', true, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_nginx::version', 'nginx_version', 'Nginx', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_varnish::plus_version', 'varnishplus_version', 'Varnish', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_ihs::version', 'ihs_version', 'IHS', true, true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_was::version', 'was_version', 'WAS', true, true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_was::wxs_version', 'wxs_version', 'WXS Client', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_wcs::version', 'wcs_version', 'WCS', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_was::jdbc_db2_version', 'jdbc_version', 'JDBC', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_wxs::version', 'wxsserver_version', 'WXS Server', true, true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_wmq::version', 'wmq_version', 'WMQ', true, true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_db2::version', 'db2_version', 'DB2', true, true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_couchbase::version', 'couchbase_version', 'CouchBase', true, true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_mongodb::org_version', 'mongodborg_version', 'MongoDB', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_kong::version', 'kong_version', 'Kong', true, true);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_redis::version', 'redis_version', 'Redis', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_rpm::kernel_date', 'last_kernel_update', 'Kernel Update', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('last_boot', 'boot_time', 'Boot Time', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('cpu::cpu_socket(s)', 'sockets', 'Sockets', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('cpu::cpu(s)', 'online_cpu', 'Online CPUs', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('lscpu::cpu(s)', 'cpu', 'All CPUs', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('lscpu::thread(s)_per_core', 'threads_per_core', 'Threads per Core', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('memory::memtotal', 'memorysize', 'Memory GB', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_df::total_gbytes', 'dfsize', 'Disk GB', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_certificate_expiration::port_443', 'certdate_expiration', 'Certificate Expiration', false, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_gitlab::version', 'gitlab_version', 'Gitlab', true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('ansible_all_ipv4_addresses', 'ipv4_addresses', 'IPv4 Addresses', true, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('ansible_dns::nameservers', 'dns_nameservers', 'DNS servers', true, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_networkmanager::default_connection_dns', 'nm_default_connection_dns', 'NetworkManager DNS', true, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_edr::service::active', 'edr_service_active', 'EDR active', true, true, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_edr::tags', 'edr_tags', 'EDR tags', true, false, false);
INSERT INTO facts (name, short_name, description, visible, aggregable, online_cpu) VALUES ('itx_facts_ansible_local::itx_facts_edr::version', 'edr_version', 'EDR version', true, true, false);
CREATE TABLE pci_subnets (
  name text primary key
);
INSERT INTO pci_subnets (name) VALUES ('${pci_vlan}');
