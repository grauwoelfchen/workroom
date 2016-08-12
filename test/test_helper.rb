ENV['RACK_ENV'] ||= 'test'

require 'pathname'
root_dir = Pathname.new(__FILE__).parent.parent.expand_path

require 'sinatra'
require 'test/unit'
require 'rack/test'
require 'nokogiri'

Dir[root_dir.join('test/support/**/*.rb')].each { |f| require f }

Sinatra::Base.set(:views, root_dir.join('src'))

app_dir = File.expand_path('../../', __FILE__)
require app_dir + '/app'
