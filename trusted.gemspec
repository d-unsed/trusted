# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'trusted/version'

Gem::Specification.new do |spec|
  spec.name          = 'trusted'
  spec.version       = Trusted::VERSION
  spec.authors       = ['Dmitry Gritsay']
  spec.email         = ['unseductable@gmail.com']

  spec.summary       = %q{Rack-compatible application server}
  spec.description   = %q{Application server for Rack apps built with Rust}
  spec.homepage      = 'https://github.com/d-unseductable/trusted'
  spec.license       = 'MIT'

  spec.extensions << 'Rakefile'

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.bindir        = 'bin'
  spec.executables   = %w(trusted)
  spec.require_paths = %w(lib)

  spec.add_dependency 'concurrent-ruby', '~> 1.0'
  spec.add_dependency 'docile', '~> 1.1'
  spec.add_dependency 'rack', '>= 1.0', '< 3.0'
  spec.add_dependency 'thermite', '~> 0.12.1'

  spec.add_development_dependency 'bundler', '~> 1.10'
  spec.add_development_dependency 'rake', '~> 10.0'
  spec.add_development_dependency 'rspec', '~> 3.5'
  spec.add_development_dependency 'rspec-its', '~> 1.2'
end
