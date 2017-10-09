require 'sinatra/base'
require 'sinatra/reloader' if development?
require 'haml'
require 'sass'

class Workroom < Sinatra::Base
  configure do
    enable :logging

    error do
      'Error :\'('
    end

    not_found do
      'Not Found :-D'
    end
  end

  helpers do
    def link_class_for(path)
      return 'active' if request.path == path
      ''
    end
  end

  get '/' do
    haml :index
  end

  %i{about software link}.map do |page|
    get "/#{page.to_s}" do
      haml page
    end
  end

  get '/stylesheet.css' do
    sass :stylesheet, :style => :compact
  end
end
