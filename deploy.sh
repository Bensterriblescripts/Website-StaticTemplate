#!/bin/sh

# End Service
systemctl stop paradisecoffee.service

# Executable
cp /home/webservice-user/Website-StaticTemplate/crm-core/target/release/crm-core /var/www/
chcon -t bin_t /var/www/crm-core

# Static Files
cp -r /home/webservice-user/Website-StaticTemplate/crm-core/templates /var/www

chown -R root:ben /var/www
chmod -R 750 /var/www

# Restart Service
systemctl start paradisecoffee.service
