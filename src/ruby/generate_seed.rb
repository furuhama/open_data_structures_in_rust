#!/usr/bin/env ruby

# File.open('seed_data.txt', 'w') do |f|
#   1_000_000.times do |i|
#     f.puts("test #{i}")
#   end
# end

RANDOM_WORDS = %w[hoge fuga piyo nyaa myon fuee waay damn].freeze

File.open('small_seed_data.txt', 'w') do |f|
  237.times do |i|
    f.puts("#{RANDOM_WORDS.sample} #{i}")
  end
end
