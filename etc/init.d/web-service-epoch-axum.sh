#!/bin/sh

### BEGIN INIT INFO
# Provides:          web-service-epoch-axum
# Required-Start:    $local_fs $network
# Required-Stop:     $local_fs
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Web service epoch axum
# Description:       Web service example that provides an epoch function that is implemented via Rust Axum
### END INIT INFO

case "$1" in
  start)
    echo "Start web-service-epoch-axum"
    PORT=10001 /opt/web-service-epoch-axum/target/release/web-service-epoch-axum
    ;;
  stop)
    echo "Stop web-service-epoch-axum"
    pgrep '[w]eb-service-epoch-axum' | xargs kill
    ;;
  *)
    echo "Usage: /etc/init.d/web-service-epoch-axum {start|stop}"
    exit 1
    ;;
esac

exit 0
