#/bin/bash

set -eo pipefail

cd ~/home/software/github/carlospeon/foreman_reports/

cd frontend
rm -f dist/assets/*.{js,css}

npm run build && cp src/assets/* dist/assets/ && cp -R dist/* ../backend/static/
tar -C dist -cf - . | ssh axinecgrafana1 sudo tar --no-same-owner -C /var/www/foreman_reports/ -xvf -

cd ../backend
cargo build -r

tar -C ~/tmp/foreman_reports/target/x86_64-unknown-linux-musl/release -cf - foreman_reports | 
  ssh axinecgrafana1 sudo tar --no-same-owner -C /usr/local/sbin -xvf -

#ssh axinecgrafana1 "sudo chmod -R o+rX /var/www/foreman_reports /usr/local/sbin/foreman_reports"
ssh axinecgrafana1 "sudo chmod -R o+rX /var/www/foreman_reports /usr/local/sbin/foreman_reports && sudo systemctl restart foreman-reports"
