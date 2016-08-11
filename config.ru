require 'rubygems'
require 'bundler'

Bundler.require

require 'rack'
require 'sinatra'

use Rack::Lint

root_dir = File.expand_path(File.dirname(__FILE__))

Sinatra::Base.set(:public_folder, File.join(root_dir, 'public'))
Sinatra::Base.set(:views,         File.join(root_dir, 'src'))
Sinatra::Base.set(:run, false)
Sinatra::Base.set(:env, ENV['RACK_ENV'] || 'development')

require root_dir + '/app'

run Workroom
