//  ===== Date Created: 01 December, 2020 ===== 

#include <string>
#include <vector>
#include "utility.hpp"

std::string day_one()
{
	std::vector<int> lines = read_file_lines("res/day_one.txt");

	for (int i = 0; i < (int) lines.size(); i++)
	{
		for (int j = i + 1; j < (int) lines.size(); j++)
		{
			if (lines[i] + lines[j] == 2020)
			{
				return std::to_string(lines[i] * lines[j]);
			}
		}
	}

	return "Unknown";
}
