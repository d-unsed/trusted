# Trusted

Rack-compatible application server. Built with

 - [Rust](https://www.rust-lang.org/)
 - [Ruru](https://github.com/d-unseductable/ruru)
 - [Thermite](https://github.com/malept/thermite)
 - [Hyper](https://github.com/hyperium/hyper)
 - [concurrent-ruby](https://github.com/ruby-concurrency/concurrent-ruby)

## Benchmarks

Application can be found
[here](https://github.com/d-unseductable/trusted_benchmark).

### Single-threaded mode

| server        | req/sec  |
| ------------- |:--------:|
| Unicorn       | 76.76    |
| Puma          | 85.50    |
| Passenger     | 85.38    |
| Trusted       | 90.83    |

### Multi-threaded mode

| server        | req/sec  |
| ------------- |:--------:|
| Puma          | 61.64    |
| Trusted       | 62.57    |

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'trusted', '~> 0.4.0'
```

And then execute:

```bash
$ bundle
```

Or install it yourself as:

```bash
$ gem install trusted
```

## Usage

```bash
$ bundle exec trusted
```

You can also use it with `rails s`

```bash
$ bundle exec rails s
```

To provide a configuration file use `-c` option

```bash
$ bundle exec trusted -c path/to/config/file.rb
```

## Configuration

If `-c` option is not provided, the default config is used

```ruby
binding_type :tcp # Use :unix for Unix sockets
listen_on '127.0.0.1:3000' # Use `/path/to/socket` for Unix sockets
native_thread_pool_size 5 # Override to change the number of native threads
rack_thread_pool_size 1 # Override to change the number of Ruby threads
```

## What's next?

- Multi-processed mode
- Signal handling
- Investigate Rails 5 crash
- Capistrano recipes
- Hijacking, WebSockets
- Optimisations, optimisations, optimisations…
- Dynamic reloading
- Windows support?

You can see an example of using Trusted in production at [this-week-in-ruru.org](http://this-week-in-ruru.org/)

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/d-unseductable/trusted. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.


## License

The gem is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
