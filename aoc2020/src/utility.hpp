//  ===== Date Created: 01 December, 2020 ===== 

#if !defined(UTILITY_HPP)
#define UTILITY_HPP

#include <vector>
#include <fstream>

std::vector<int> read_file_lines(const char* filename)
{
	std::vector<int> lines;
	std::string line;
	std::ifstream file { filename };

	if (!file)
	{
		printf("Error: Input file does not exists.\n");
		return lines;
	}

	while (std::getline(file, line))
	{
		lines.emplace_back(std::stoi(line));
	}

	return lines;
}

#endif
