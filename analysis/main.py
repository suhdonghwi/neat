import re


class Specie:
    def __init__(self, data):
        pat = re.compile(
            r"""\s+(\d+)\s+\|\s+(\d+)\s+\|\s+(\d+)\s+\|\s+(\d+)\s+\|\s+(\d+[.]\d+)"""
        )
        (id, age, size, offspring, fitness) = pat.match(data).groups()

        self.id = int(id)
        self.age = int(age)
        self.size = int(size)
        self.offspring = int(offspring)
        self.fitness = float(fitness)


class Generation:
    def __init__(self, data):
        data = data.strip()

        pat = re.compile(
            r"""\[Generation (\d+)\]\n# Evaluation result\n  - fitness max: (\d+\.\d+) \((\d+) nodes, (\d+) edges\)\n  - fitness mean: (\d+\.\d+) .+\n\n# Speciation result:\n.+\n.+\n([\s\S]+)"""
        )
        (
            generation_num,
            fitness_max,
            nodes,
            edges,
            fitness_mean,
            species_data,
        ) = pat.match(data).groups()

        self.generation_num = int(generation_num)
        self.fitness_max = float(fitness_max)
        self.best_nodes_count = int(nodes)
        self.best_edges_count = int(edges)
        self.fitness_mean = float(fitness_mean)

        self.species = [Specie(l) for l in species_data.split("\n")]
        print(self.species)


file = open("./analysis/output.txt", "r")

generations = [
    Generation(g) for g in re.compile("[-]+\n").split(file.read()) if g.strip() != ""
]
print(generations)
