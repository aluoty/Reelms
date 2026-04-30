#include <algorithm>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <random>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

struct Fish {
  std::string species;
  std::string rarity;
  double weightKg;
  int value;
};

int extractFishCaughtFromProgress(const std::string& path) {
  std::ifstream input(path);
  if (!input.is_open()) {
    return 0;
  }

  std::ostringstream buffer;
  buffer << input.rdbuf();
  const std::string content = buffer.str();

  std::regex fishCaughtRegex("\"fishCaught\"\\s*:\\s*(\\d+)");
  std::smatch match;
  if (std::regex_search(content, match, fishCaughtRegex) && match.size() > 1) {
    return std::stoi(match[1].str());
  }

  return 0;
}

int rarityScore(const std::string& rarity) {
  if (rarity == "Common") return 1;
  if (rarity == "Uncommon") return 2;
  if (rarity == "Rare") return 3;
  if (rarity == "Epic") return 4;
  if (rarity == "Legendary") return 5;
  if (rarity == "Mythic") return 6;
  return 1;
}

std::string rarityFromRoll(int roll) {
  if (roll <= 40) return "Common";
  if (roll <= 67) return "Uncommon";
  if (roll <= 84) return "Rare";
  if (roll <= 95) return "Epic";
  if (roll <= 99) return "Legendary";
  return "Mythic";
}

int main(int argc, char* argv[]) {
  std::random_device rd;
  std::mt19937 gen(rd());

  int fishCaught = 0;
  std::string location = "Pond";
  int rodLevel = 1;
  int baitLevel = 1;
  std::string weather = "Clear";
  std::string moon = "Normal";
  int luckBuffCasts = 0;
  int valueBuffCasts = 0;

  if (argc > 1) {
    fishCaught = extractFishCaughtFromProgress(argv[1]);
  }
  if (argc > 2) location = argv[2];
  if (argc > 3) rodLevel = std::max(1, std::stoi(argv[3]));
  if (argc > 4) baitLevel = std::max(1, std::stoi(argv[4]));
  if (argc > 5) weather = argv[5];
  if (argc > 6) moon = argv[6];
  if (argc > 7) luckBuffCasts = std::max(0, std::stoi(argv[7]));
  if (argc > 8) valueBuffCasts = std::max(0, std::stoi(argv[8]));

  std::vector<std::string> speciesPool = {"Carp", "Salmon", "Trout", "Catfish", "Pike"};
  std::vector<std::string> locationExotics;
  double weightMin = 0.4;
  double weightMax = 11.0;

  if (location == "River") {
    speciesPool = {"Carp", "Salmon", "Trout", "Catfish", "Pike", "Bass"};
    locationExotics = {"GoldenKoi", "SilverEel"};
    weightMin = 0.8;
    weightMax = 15.0;
  } else if (location == "Ocean") {
    speciesPool = {"Tuna", "Mackerel", "Swordfish", "Marlin", "Bass"};
    locationExotics = {"BluefinTitan", "Glowfin"};
    weightMin = 1.5;
    weightMax = 30.0;
  } else if (location == "VolcanicBay") {
    speciesPool = {"LavaSnapper", "AshGrouper", "Tuna", "Marlin"};
    locationExotics = {"MagmaRay", "PhoenixKoi"};
    weightMin = 2.5;
    weightMax = 35.0;
  }

  std::uniform_int_distribution<int> rarityRoll(1, 100);
  std::uniform_int_distribution<int> speciesDist(
      0, static_cast<int>(speciesPool.size() - 1));
  std::uniform_real_distribution<double> weightDist(weightMin, weightMax);
  std::uniform_int_distribution<int> exoticChance(1, 100);

  // Better rods/bait/buffs and moon/weather affect rarity.
  int rarityShift = 0;
  rarityShift += std::min(fishCaught / 25, 8);  // progression
  rarityShift += std::max(0, rodLevel - 1);
  rarityShift += std::max(0, baitLevel - 1);
  rarityShift += luckBuffCasts > 0 ? 3 : 0;
  if (weather == "Storm") rarityShift += 2;
  if (weather == "Rain") rarityShift += 1;
  if (moon == "BlueMoon") rarityShift += 7;
  if (moon == "BloodMoon") rarityShift -= 7;

  int roll = std::max(1, std::min(100, rarityRoll(gen) - rarityShift));
  std::string rarity = rarityFromRoll(roll);

  std::string species = speciesPool[speciesDist(gen)];
  if (!locationExotics.empty()) {
    int chance = 6;
    if (location == "Ocean") chance = 8;
    if (location == "VolcanicBay") chance = 12;
    chance += baitLevel - 1;
    chance += (moon == "BlueMoon") ? 5 : 0;
    chance -= (moon == "BloodMoon") ? 3 : 0;
    if (exoticChance(gen) <= std::max(2, chance)) {
      std::uniform_int_distribution<int> exoticDist(
          0, static_cast<int>(locationExotics.size() - 1));
      species = locationExotics[exoticDist(gen)];
      if (rarityScore(rarity) < rarityScore("Epic")) {
        rarity = "Epic";
      }
    }
  }

  Fish fish;
  fish.species = species;
  fish.rarity = rarity;
  fish.weightKg = weightDist(gen);

  double rarityMultiplier = 1.0;
  if (fish.rarity == "Uncommon") rarityMultiplier = 1.5;
  if (fish.rarity == "Rare") rarityMultiplier = 2.4;
  if (fish.rarity == "Epic") rarityMultiplier = 4.0;
  if (fish.rarity == "Legendary") rarityMultiplier = 7.5;
  if (fish.rarity == "Mythic") rarityMultiplier = 10.0;

  double locationMultiplier = 1.0;
  if (location == "River") locationMultiplier = 1.2;
  if (location == "Ocean") locationMultiplier = 1.55;
  if (location == "VolcanicBay") locationMultiplier = 2.1;

  double weatherMultiplier = 1.0;
  if (weather == "Storm") weatherMultiplier = 1.15;
  if (moon == "BlueMoon") weatherMultiplier *= 1.30;
  if (moon == "BloodMoon") weatherMultiplier *= 0.80;
  if (valueBuffCasts > 0) weatherMultiplier *= 1.20;

  fish.value = static_cast<int>(fish.weightKg * 18.0 * rarityMultiplier *
                                locationMultiplier * weatherMultiplier);

  // Output format is intentionally machine-friendly for Java:
  // species|rarity|weightKg|value
  std::cout << fish.species << "|" << fish.rarity << "|"
            << std::fixed << std::setprecision(2) << fish.weightKg << "|"
            << fish.value << std::endl;

  return 0;
}
