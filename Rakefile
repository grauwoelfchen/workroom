require 'rake/testtask'

task :default => :test

dir = File.dirname(__FILE__)
Rake::TestTask.new do |t|
  t.libs << 'test'
  t.test_files = Dir.glob("#{dir}/test/**/*_test.rb")
  t.warning = false
  t.verbose = false
end
