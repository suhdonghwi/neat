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
            if gen.fitness_max >= 3.9:
                succ_gens.append(i)
                break

    print("Success : " + str(len(succ_gens)))
    sns.histplot(succ_gens, kde=True)


def plot_size(cases):
    sizes = []
    for gens in cases:
        for (i, gen) in enumerate(gens):
            if gen.fitness_max >= 3.9:
                sizes.append(gen.best_edges_count)
                break

    print("Mean : " + str(np.array(sizes).mean()))
    sns.countplot(x=sizes)


case1 = split_cases("./analysis/output.txt")
case2 = split_cases("./analysis/output-no.txt")

cases = [case1, case2]
labels = ["With destructive mutation", "Without destructive mutation"]

for case in cases:
    plot_fitness_max(case)
    plt.show()


for case, label in zip(cases, labels):
    plot_succ_gens(case)

    plt.title("survival rate = " + label)
    plt.xlabel("Generation")
    plt.show()


for case, label in zip(cases, labels):
    plot_size(case)

    plt.title("survival rate = " + label)
    plt.xlabel("Size")
    plt.show()
