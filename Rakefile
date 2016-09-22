require 'bundler/gem_tasks'
require 'thermite/tasks'
require 'rspec/core/rake_task'

RSpec::Core::RakeTask.new(:spec)
Thermite::Tasks.new(cargo_project_path: 'ext/trusted')

task default: %w(thermite:build)
