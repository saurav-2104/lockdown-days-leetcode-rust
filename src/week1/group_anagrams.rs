/**
	Given an array of strings, group anagrams together.

	Example:

	Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
	Output:
	[
	["ate","eat","tea"],
	["nat","tan"],
	["bat"]
	]
*/
use std::collections::HashMap;

impl Solution {
	pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
		let mut map: HashMap<String, Vec<String>> = HashMap::new();

		for s in strs {
			let mut key: Vec<char> = s.chars().collect();
			key.sort();
			let sorted_key = key.iter().collect::<String>();
			map.entry(sorted_key).or_insert(vec![]).push(s);
		}

		return map.into_iter().map(|(_, v)| v).collect();
	}
}
