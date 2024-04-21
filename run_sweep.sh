#!/bin/bash
functions="ackley,schwefel,brown,rastrigin,schwefel2,solomon"

runs_per_set=64

# bats
bat_count=20
bat_iters=1000
frequency_left_bound=0
frequency_right_bound=1
initial_pulse_rate=0.7
initial_loudness=1.4

pulse_rate_factors=(0.1 0.3 0.5 0.7 0.9)
loudness_cooling_rates=(0.1 0.3 0.5 0.7 0.9)

# butterflies
butterfly_count=20
butterfly_iters=1000
fragrance_exponent_left_bound=0.1 # default as per slide
fragrance_exponent_right_bound=0.3

fragrance_multipiers=(0.1 0.3 0.5 0.7 0.9)
local_search_chances=(0.1 0.3 0.5 0.7 0.9)

rm -rf output_bats
rm -rf output_butterflies
mkdir -p output_bats
mkdir -p output_butterflies

cargo build --release

# process bats
for pulse_rate_factor in "${pulse_rate_factors[@]}"
do
	for loudness_cooling_rate in "${loudness_cooling_rates[@]}"
	do
		./target/release/swarm_optimizers --functions=$functions --try-count $runs_per_set bats --bat-count $bat_count \
			--bat-num-iters $bat_iters --frequency-left-bound $frequency_left_bound --frequency-right-bound $frequency_right_bound \
			--initial-pulse-rate $initial_pulse_rate --initial-loudness $initial_loudness --pulse-rate-factor $pulse_rate_factor \
			--loudness-cooling-rate $loudness_cooling_rate > "output_bats/ratefactor_"$pulse_rate_factor"_coolingrate_"$loudness_cooling_rate".txt"
	done
done

# process butterflies
for fragrance_multipier in "${fragrance_multipiers[@]}"
do
	for local_search_chance in "${local_search_chances[@]}"
	do
		./target/release/swarm_optimizers --functions=$functions --try-count $runs_per_set butterflies --butterfly-count $butterfly_count \
			--butterfly-num-iters $butterfly_iters --fragrance-exponent-left-bound $fragrance_exponent_left_bound --fragrance-exponent-right-bound $fragrance_exponent_right_bound \
			--fragrance-multiplier $fragrance_multipier --local-search-chance $local_search_chance > "output_butterflies/multiplier_"$fragrance_multipier"_searchchance_"$local_search_chance".txt"
	done
done