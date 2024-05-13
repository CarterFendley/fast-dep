from cProfile import Profile
from pstats import Stats, SortKey
from typing import Optional

class Timer:
    def __init__(self):
        self.latest_profile: Optional[Profile] = None
        self.latest_stats: Optional[Stats] = None

    @property
    def latest_cumtime(self) -> float:
        total_time = 0.0
        for func, value in self.latest_stats.stats.items():
            (_, _, cumulative_time, _, _) = value
            total_time += cumulative_time

        return total_time

    def print_latest(self):
        self.latest_stats.print_stats()

    def __enter__(self):
        self.latest_profile = Profile()
        self.latest_stats = None

        self.latest_profile.enable()

    def __exit__(self, *exc_info):
        self.latest_profile.disable()
        self.latest_stats = Stats(
            self.latest_profile
        ).sort_stats(SortKey.CUMULATIVE)
