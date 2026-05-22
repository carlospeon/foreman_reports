DROP MATERIALIZED VIEW openscap_report;
CREATE MATERIALIZED VIEW IF NOT EXISTS openscap_report as
select
  'digital' as bu,
  h.id as id,
  h.name as hostname,
  t.name as location,
  e.name as environment,
  f_os.value as os,
  lr.reported_at as reported_at,
  jsonb_build_object(
    (string_to_array((string_to_array(lr.metrics, E'\n'))[2],': '))[1],
      (string_to_array((string_to_array(lr.metrics, E'\n'))[2],': '))[2],
    (string_to_array((string_to_array(lr.metrics, E'\n'))[3],': '))[1],
      (string_to_array((string_to_array(lr.metrics, E'\n'))[3],': '))[2],
    (string_to_array((string_to_array(lr.metrics, E'\n'))[4],': '))[1],
      (string_to_array((string_to_array(lr.metrics, E'\n'))[4],': '))[2]
  ) as metrics,
  jrl.result as result,
  h.comment as comment
from digital.hosts h 
  inner join digital.taxonomies t
    on h.location_id = t.id
  inner join digital.katello_content_facets kcf
    on h.id = kcf.host_id
  inner join digital.katello_content_view_environment_content_facets kcvecf
    on kcf.id = kcvecf.content_facet_id
  inner join digital.katello_content_view_environments kcve 
    on kcvecf.content_view_environment_id = kcve.id
  inner join digital.katello_environments e 
    on kcve.environment_id = e.id
  left join digital.fact_values f_os
    on f_os.fact_name_id = (select id from digital.fact_names where name = 'distribution::version')
    and h.id = f_os.host_id
  inner join lateral (
    select * from digital.reports r
    where
      r.host_id = h.id and
      r.type='ForemanOpenscap::ArfReport'
    order by reported_at desc
    limit 1
  ) lr on true
  inner join lateral (
    select l.report_id, json_object_agg(s.value, l.result) as result
    from digital.logs l  
      inner join digital.sources s on
        s.id = l.source_id
    where l.report_id = lr.id
    group by l.report_id
  ) jrl on true

union all
select
  'sscc' as bu,
  h.id as id,
  h.name as hostname,
  t.name as location,
  e.name as environment,
  f_os.value as os,
  lr.reported_at as reported_at,
  jsonb_build_object(
    (string_to_array((string_to_array(lr.metrics, E'\n'))[2],': '))[1],
      (string_to_array((string_to_array(lr.metrics, E'\n'))[2],': '))[2],
    (string_to_array((string_to_array(lr.metrics, E'\n'))[3],': '))[1],
      (string_to_array((string_to_array(lr.metrics, E'\n'))[3],': '))[2],
    (string_to_array((string_to_array(lr.metrics, E'\n'))[4],': '))[1],
      (string_to_array((string_to_array(lr.metrics, E'\n'))[4],': '))[2]
  ) as metrics,
  jrl.result as result,
  h.comment as comment
from sscc.hosts h 
  inner join sscc.taxonomies t
    on h.location_id = t.id
  inner join sscc.katello_content_facets kcf
    on h.id = kcf.host_id
  inner join sscc.katello_content_view_environment_content_facets kcvecf
    on kcf.id = kcvecf.content_facet_id
  inner join sscc.katello_content_view_environments kcve 
    on kcvecf.content_view_environment_id = kcve.id
  inner join sscc.katello_environments e 
    on kcve.environment_id = e.id
  left join sscc.fact_values f_os
    on f_os.fact_name_id = (select id from sscc.fact_names where name = 'distribution::version')
    and h.id = f_os.host_id
  inner join lateral (
    select * from sscc.reports r
    where
      r.host_id = h.id and
      r.type='ForemanOpenscap::ArfReport'
    order by reported_at desc
    limit 1
  ) lr on true
  inner join lateral (
    select l.report_id, json_object_agg(s.value, l.result) as result
    from sscc.logs l  
      inner join sscc.sources s on
        s.id = l.source_id
    where l.report_id = lr.id
    group by l.report_id
  ) jrl on true
with data;

CREATE UNIQUE INDEX openscap_report_id on openscap_report(bu, id);
grant select on openscap_report to foreman_reports;
insert into materialized_views (name, refreshed_at) values ('openscap_report', CURRENT_TIMESTAMP);


