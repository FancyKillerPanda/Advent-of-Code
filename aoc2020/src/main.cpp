//  ===== Date Created: 29 November, 2020 ===== 

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <vector>
#include <string>

#include "day_one.cpp"
#include "day_two.cpp"

const std::vector<std::string(*)()> days {
	day_one_part_one, day_one_part_two,
	day_two_part_one, day_two_part_two,
};

int main(int argc, char* argv[])
{
	if (argc > 1)
	{
		if (strcmp(argv[1], "--all") == 0)
		{
			for (int i = 0; i < (int) days.size(); i += 2)
			{
					printf("Running day %d:\nResult (part one): %s\nResult (part two): %s\n\n",
						   i / 2,
						   days[i]().c_str(),
						   days[i + 1]().c_str());
			}
		}
		else
		{
			for (int i = 1; i < argc; i++)
			{
				int dayNumber = atoi(argv[i]);

				if (!dayNumber)
				{
					printf("Error: Could not run day '%s'.\n", argv[i]);
				}
				else if (dayNumber > (int) days.size() / 2)
				{
					printf("Error: Day %d has not yet been completed.\n", dayNumber);
				}
				else
				{
					printf("Running day %d:\nResult (part one): %s\nResult (part two): %s\n\n",
						   dayNumber,
						   days[(dayNumber - 1) / 2]().c_str(),
						   days[(dayNumber - 1) / 2 + 1]().c_str());
				}
			}
		}
	}
	else
	{
		if (days.size() == 0)
		{
			printf("Error: No days have yet been completed. Come back later!\n");
		}
		else
		{
			printf("Running latest day (day %d):\nResult (part one): %s\nResult (part two): %s\n",
				   (int) days.size() / 2,
				   days[days.size() - 2]().c_str(),
				   days[days.size() - 1]().c_str());
		}
	}
}
