#!/usr/bin/env python3
"""Generate JSON fixtures from BO4E-Python for golden file testing.

This script generates JSON fixtures that match the BO4E-Python library output.
The fixtures are used for compatibility testing with the Rust implementation.

Usage:
    cd tests/golden && python generate.py

Note: Requires bo4e package: pip install bo4e
"""

import json
from pathlib import Path

# Try to import BO4E Python library
try:
    from bo4e import Zaehler, Marktlokation, Geschaeftspartner
    from bo4e.enum import Sparte, Energierichtung, Geschaeftspartnerrolle
    HAS_BO4E = True
except ImportError:
    HAS_BO4E = False
    print("Warning: bo4e package not installed. Using hardcoded fixtures.")

FIXTURES_DIR = Path(__file__).parent / "fixtures"
FIXTURES_DIR.mkdir(exist_ok=True)


def write_fixture(name: str, data: dict):
    """Write a fixture as JSON."""
    path = FIXTURES_DIR / f"{name}.json"
    with open(path, "w") as f:
        json.dump(data, f, indent=2)
    print(f"Wrote {path}")


def generate_with_bo4e():
    """Generate fixtures using the BO4E Python library."""
    # Meter fixture
    meter = Zaehler(
        zaehlernummer="1EMH0012345678",
        sparte=Sparte.STROM,
    )
    write_fixture("meter", meter.model_dump(by_alias=True, exclude_none=True))

    # MarketLocation fixture
    malo = Marktlokation(
        marktlokationsId="12345678901",
        sparte=Sparte.STROM,
        energierichtung=Energierichtung.VERBRAUCH,
    )
    write_fixture("market_location", malo.model_dump(by_alias=True, exclude_none=True))

    # BusinessPartner fixture
    partner = Geschaeftspartner(
        name1="Stadtwerke Musterstadt GmbH",
        geschaeftspartnerrollen=[Geschaeftspartnerrolle.LIEFERANT],
    )
    write_fixture("business_partner", partner.model_dump(by_alias=True, exclude_none=True))


def generate_hardcoded():
    """Generate hardcoded fixtures that match BO4E Python format."""
    # Meter fixture - matches Python output format
    write_fixture("meter", {
        "_typ": "Zaehler",
        "_version": "202401.4.1",
        "zaehlernummer": "1EMH0012345678",
        "sparte": "STROM"
    })

    # MarketLocation fixture - Note: Python uses AUSSP/EINSP for energy direction
    write_fixture("market_location", {
        "_typ": "Marktlokation",
        "_version": "202401.4.1",
        "marktlokationsId": "12345678901",
        "sparte": "STROM",
        "energierichtung": "AUSSP"  # FeedOut in Rust
    })

    # BusinessPartner fixture
    write_fixture("business_partner", {
        "_typ": "Geschaeftspartner",
        "_version": "202401.4.1",
        "name1": "Stadtwerke Musterstadt GmbH",
        "geschaeftspartnerrollen": ["LIEFERANT"]
    })


def main():
    if HAS_BO4E:
        generate_with_bo4e()
    else:
        generate_hardcoded()
    print("Done generating fixtures")


if __name__ == "__main__":
    main()
