require 'test_helper'

class AppTest < Test::Unit::TestCase
  include Rack::Test::Methods

  def app
    Workroom
  end

  must('return status 200') do
    get('/')
    assert(last_response.ok?)
    assert_equal(200, last_response.status)
  end

  must('have a title') do
    get('/')
    html = Nokogiri::XML(last_response.body)
    assert_equal("Grauwoelfchen's Workroom", html.at('title').text)
  end

  must('have a my name') do
    get('/about')
    html = Nokogiri::XML(last_response.body)
    assert_equal('Yasuhiro Asaka', html.at('p').text)
  end
end
