import re
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns


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
            r"""\[Generation (\d+)\]\n# Evaluation result\n  - fitness max: (\d+\.?\d*) \((\d+) nodes, (\d+) edges\)\n  - fitness mean: (\d+\.?\d*) .+\n\n# Speciation result:\n.+\n.+\n([\s\S]+)"""
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


def parse_generations(content):
    splited = re.compile("[-]+\n").split(content)

    return [Generation(g) for g in splited if g.strip() != ""]


def split_cases(path):
    result = []

    file = open(path, "r")
    content = file.read()
    splited = content.split("<Case Start>")
    for data in splited:
        gens = parse_generations(data)
        result.append(gens)

    return result


def plot_fitness_max(cases):
    for gens in cases:
        plt.plot([x.fitness_max for x in gens])

    plt.xlabel("Generation")
    plt.ylabel("Fitness")


def plot_succ_gens(cases):
    succ_gens = []
    for gens in cases:
        for (i, gen) in enumerate(gens):
            if gen.fitness_max >= 3.95:
                succ_gens.append(i)
                break

    print("Success : " + str(len(succ_gens)))
    sns.kdeplot(succ_gens)


def plot_size(cases):
    sizes = []
    for gens in cases:
        for (i, gen) in enumerate(gens):
            if gen.fitness_max >= 3.95:
                sizes.append(gen.best_edges_count)
                break

    print("Success : " + str(len(sizes)))
    sns.kdeplot(sizes)


case5 = split_cases("./analysis/output-3.txt")
case8 = split_cases("./analysis/output-8.txt")
case15 = split_cases("./analysis/output-15.txt")
case30 = split_cases("./analysis/output-30.txt")

plot_fitness_max(case5)
plt.show()

plot_fitness_max(case30)
plt.show()


plot_succ_gens(case5)
# plot_succ_gens(case8)
# plot_succ_gens(case15)
plot_succ_gens(case30)

plt.legend(labels=["5", "8", "15", "30"])
plt.xlabel("Generation")
plt.title("Success generation")

plt.show()

plot_size(case5)
# plot_size(case8)
# plot_size(case15)
plot_size(case30)

plt.legend(labels=["5", "8", "15", "30"])
plt.xlabel("Size")
plt.title("Success network complexity")

plt.show()
