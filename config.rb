# puma config file

env_values = ['production', 'development']

app_dir = File.expand_path(__dir__)

app_env = ENV['RACK_ENV']
app_env = 'development' unless env_values.include?(app_env)

daemonize true
environment app_env
threads 0, 16
workers 0
pidfile "#{app_dir}/tmp/pids/workroom.pid"
bind "unix:///#{app_dir}/tmp/sockets/workroom.sock"
stdout_redirect "#{app_dir}/tmp/logs/#{app_env}.log"
preload_app!
