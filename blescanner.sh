PID=blescanner.pid
DATABASE_URL=postgres://mbasa:mbasa@localhost/ble

sig () {
  test -s "$PID" && kill -$1 `cat $PID` && rm -f $PID
}

case "$1" in
  start)
    echo "Starting blescanner app ..."
     echo "DATABASE_URL=$DATABASE_URL" > .env;cargo run --release  &
    _pid=$!
    echo "$_pid" > $PID
    echo "blescanner app started!"
    ;;
  stop)
    echo "Stoping blescanner app ..."
    sig QUIT && exit 0
    echo "Not running"
    ;;
  restart)
    ;;
  status)
    ;;
  *)
    echo "Usage: $0 {start|stop|restart|status}"
    ;;
esac
