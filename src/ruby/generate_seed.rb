#!/usr/bin/env ruby

File.open('seed_data.txt', 'w') do |f|
  1_000_000.times do |i|
    f.puts("test #{i}")
  end
end
