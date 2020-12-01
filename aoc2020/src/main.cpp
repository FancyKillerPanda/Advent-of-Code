//  ===== Date Created: 29 November, 2020 ===== 

#include <stdlib.h>
#include <stdio.h>
#include <vector>

const std::vector<void(*)()> days;

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
				printf("Running day %d:\n", dayNumber);
				days[dayNumber - 1]();
				printf("\n");
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
			printf("Running latest day (%d):\n", (int) days.size());
			days.back()();
			printf("\n");
		}
	}
}
