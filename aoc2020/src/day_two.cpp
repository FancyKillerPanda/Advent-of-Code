//  ===== Date Created: 02 December, 2020 ===== 

#include <string>
#include <vector>
#include <sstream>

#include "utility.hpp"

std::string day_two_part_one()
{
	std::vector<std::string> lines = read_file_lines("res/day_two.txt");
	unsigned int validPasswords = 0;

	for (const std::string& line : lines)
	{
		std::stringstream lineStream { line };
		unsigned int minimum = 0;
		unsigned int maximum = 0;
		char letter;
		std::string restOfLine;
		char unused[2];

		lineStream >> minimum;
		lineStream.read(unused, 1);
		lineStream >> maximum;
		lineStream.read(unused, 1);
		lineStream >> letter;
		lineStream.read(unused, 2);
		lineStream >> restOfLine;

		unsigned int count = 0;
		
		for (char character : restOfLine)
		{
			if (character == letter)
			{
				count += 1;
			}
		}

		if (count >= minimum && count <= maximum)
		{
			validPasswords += 1;
		}
	}

	return std::to_string(validPasswords);
}

std::string day_two_part_two()
{
	std::vector<std::string> lines = read_file_lines("res/day_two.txt");
	unsigned int validPasswords = 0;

	for (const std::string& line : lines)
	{
		std::stringstream lineStream { line };
		unsigned int firstPos = 0;
		unsigned int secondPos = 0;
		char letter;
		std::string restOfLine;
		char unused[2];

		lineStream >> firstPos;
		lineStream.read(unused, 1);
		lineStream >> secondPos;
		lineStream.read(unused, 1);
		lineStream >> letter;
		lineStream.read(unused, 2);
		lineStream >> restOfLine;

		if (restOfLine[firstPos - 1] == letter ^ restOfLine[secondPos - 1] == letter)
		{
			validPasswords += 1;
		}
	}
	
	return std::to_string(validPasswords);
}
