//  ===== Date Created: 29 November, 2020 ===== 

#include <stdlib.h>
#include <stdio.h>
#include <vector>
#include <string>

#include "day_one.cpp"

const std::vector<std::string(*)()> days = { day_one };

int main(int argc, char* argv[])
{
	if (argc > 1)
	{
		for (int i = 1; i < argc; i++)
		{
			int dayNumber = atoi(argv[i]);

			if (!dayNumber)
			{
				printf("Error: Could not run day '%s'.\n", argv[i]);
			}
			else if (dayNumber > (int) days.size())
			{
				printf("Error: Day %d has not yet been completed.\n", dayNumber);
			}
			else
			{
				printf("Running day %d:\nResult: %s\n\n", dayNumber, days[dayNumber - 1]().c_str());
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
			printf("Running latest day (%d):\nResult: %s\n\n", (int) days.size(), days.back()().c_str());
		}
	}
}
