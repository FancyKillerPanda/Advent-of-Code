//  ===== Date Created: 03 December, 2020 ===== 

#include <string>
#include <vector>
#include "utility.hpp"

std::string day_five_part_one()
{
	std::vector<std::string> lines = read_file_lines("res/day_five.txt");
	int highestId = 0;

	for (const std::string& line : lines)
	{
		int min = 0;
		int max = 127;
		int row = 0;
		int col = 0;
		
		for (int i = 0; i < 7; i++)
		{
			if (line[i] == 'F')
			{
				max -= ((max - min) / 2) + 1;
			}
			else
			{
				min += ((max - min) / 2) + 1;
			}
		}
		
		row = min; // Should be same as max
		
		min = 0;
		max = 7;
		
		for (int i = 7; i < 10; i++)
		{
			if (line[i] == 'L')
			{
				max -= ((max - min) / 2) + 1;
			}
			else
			{
				min += ((max - min) / 2) + 1;
			}
		}

		col = min; // Should be same as max
		int id = row * 8 + col;

		if (id > highestId)
		{
			highestId = id;
		}
	}
	
	return std::to_string(highestId);
}

std::string day_five_part_two()
{
	std::vector<std::string> lines = read_file_lines("res/day_five.txt");
	return "Unknown";
}
