//  ===== Date Created: 03 December, 2020 ===== 

#include <string>
#include <vector>
#include "utility.hpp"

std::string day_four_part_one()
{
	std::vector<std::string> lines = read_file_lines("res/day_four.txt");
	
	bool hasBirthYear = false;
	bool hasIssueYear = false;
	bool hasExpirationYear = false;
	bool hasHeight = false;
	bool hasHairColour = false;
	bool hasEyeColour = false;
	bool hasPassportId = false;
	bool hasCountryId = false;

	unsigned int validPassports = 0;
	
	for (const std::string& line : lines)
	{
		if (line == "")
		{
			if (hasBirthYear && hasIssueYear && hasExpirationYear &&
				hasHeight && hasHairColour && hasEyeColour &&
				hasPassportId)
			{
				validPassports += 1;
			}
			
			hasBirthYear = hasIssueYear = hasExpirationYear =
				hasHeight = hasHairColour = hasEyeColour =
				hasPassportId = hasCountryId = false;
			continue;
		}
		
		if (line.find("byr:") != std::string::npos) hasBirthYear = true;
		if (line.find("iyr:") != std::string::npos) hasIssueYear = true;
		if (line.find("eyr:") != std::string::npos) hasExpirationYear = true;
		if (line.find("hgt:") != std::string::npos) hasHeight = true;
		if (line.find("hcl:") != std::string::npos) hasHairColour = true;
		if (line.find("ecl:") != std::string::npos) hasEyeColour = true;
		if (line.find("pid:") != std::string::npos) hasPassportId = true;
		if (line.find("cid:") != std::string::npos) hasCountryId = true;
	}

	return std::to_string(validPassports);
}

std::string day_four_part_two()
{
	std::vector<std::string> lines = read_file_lines("res/day_four.txt");
	return "Unknown";
}
