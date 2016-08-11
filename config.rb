# puma config file

app_dir = ENV['APP_DIR']
app_env = ENV['RACK_ENV'] || 'development'

daemonize true
environment app_env
threads 0, 16
workers 0
pidfile "#{app_dir}/tmp/pids/workroom.pid"
bind "unix:///#{app_dir}/tmp/sockets/wookroom.sock"
stdout_redirect "#{app_dir}/tmp/logs/#{app_env}.log"
preload_app!
