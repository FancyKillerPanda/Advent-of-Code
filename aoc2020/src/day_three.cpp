//  ===== Date Created: 03 December, 2020 ===== 

#include <string>
#include <vector>
#include "utility.hpp"

std::string day_three_part_one()
{
	std::vector<std::string> lines = read_file_lines("res/day_three.txt");

	int col = 0;
	int numberOfTrees = 0;
	
	for (int row = 0; row < (int) lines.size(); row++)
	{
		const std::string& line = lines[row];

		if (line[col % line.size()] == '#')
		{
			numberOfTrees += 1;
		}

		col += 3;
	}
	
	return std::to_string(numberOfTrees);
}

std::string day_three_part_two()
{
	std::vector<std::string> lines = read_file_lines("res/day_three.txt");

	unsigned long long multipliedNumberOfTrees = 1;
	std::vector<int> colSlopes = { 1, 3, 5, 7, 1 };

	for (int i = 0; i < (int) colSlopes.size(); i++)
	{
		int col = 0;
		unsigned long long numberOfTrees = 0;
		
		for (int row = 0; row < (int) lines.size(); row++)
		{
			const std::string& line = lines[row];

			if (line[col % line.size()] == '#')
			{
				numberOfTrees += 1;
			}

			col += colSlopes[i];

			if (i + 1 == (int) colSlopes.size())
			{
				row += 1;
			}
		}

		multipliedNumberOfTrees *= numberOfTrees;
	}
	
	return std::to_string(multipliedNumberOfTrees);
}
