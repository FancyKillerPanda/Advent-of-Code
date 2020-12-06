//  ===== Date Created: 03 December, 2020 ===== 

#include <string>
#include <vector>
#include <algorithm>
#include "utility.hpp"

std::vector<int> getIds()
{
	std::vector<std::string> lines = read_file_lines("res/day_five.txt");
	std::vector<int> idList;

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
		idList.emplace_back(row * 8 + col);
	}

	return idList;
}

std::string day_five_part_one()
{
	int highestId = 0;
	
	for (int id : getIds())
	{
		if (id > highestId)
		{
			highestId = id;
		}
	}
	
	return std::to_string(highestId);
}

std::string day_five_part_two()
{
	int ourId = 0;
	std::vector<int> idList = getIds();
	std::sort(idList.begin(), idList.end(), [](int first, int second) { return first < second; });

	for (int i = 1; i < (int) idList.size(); i++)
	{
		if (idList[i - 1] + 2 == idList[i])
		{
			ourId = idList[i - 1] + 1;
			break;
		}
	}
	
	return std::to_string(ourId);
}
