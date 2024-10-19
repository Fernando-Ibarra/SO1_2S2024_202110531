FROM grafana/grafana

ADD ./provisioning /etc/grafana/provisioning

# Set environment variables
ENV GF_SECURITY_ADMIN_PASSWORD=admin
ENV GF_SECURITY_ADMIN_USER=admin

# install plugins (redis)
ENV GF_INSTALL_PLUGINS=redis-datasource

# Expose the port
EXPOSE 3000