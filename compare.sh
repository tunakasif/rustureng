#!/bin/bash

# if hyperfine command is not installed exit
if ! command -v hyperfine &> /dev/null; then
    echo "hyperfine command could not be found";
    exit;
fi

args=("$@")
search_term=$(printf "%s " "${args[@]}");
search_term=${search_term%?};
if [ -z "$search_term" ]; then
    search_term="test";
fi
search_term_url=${search_term// /%20};

# parameters
user_agent='MyAgent';
warmup_count=5;
min_runs=10;

# run hyperfine compare
cargo build --release || exit;
rustureng_command="./target/release/rustureng $search_term";
curl_command="curl -s -o /dev/null 'https://tureng.com/en/turkish-english/$search_term_url' -H 'User-Agent: $user_agent'";
hyperfine "$curl_command" "$rustureng_command" --warmup $warmup_count --min-runs $min_runs

