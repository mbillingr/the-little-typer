from dataclasses import dataclass


class Neutral: pass


@dataclass
class NVar:
    name: str
