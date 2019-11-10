# Dynasties

**Dynasties** is a procedural family tree generator

## Family

A family is a collection of `Person`'s who are all directly or indirectly related to a family *`root`*.

A family is initialized by generating a *core*. A core is composed of a `root` and its two *back-generated* parents. The root is the only person in a family to be generated this way. Afterwards, the family is simulated season by season by going through every active person in the collection and simulating one season of their lives.

```mermaid
graph LR
    p1[Parent 1]
    p2[Parent 2]
    r[root]

    p1 --- r
    p2 --- r
    r --> ...

    classDef reg fill: #0000, stroke: #fff, color: #fff
    class r,p1,p2,... reg
```

Potentially, a family could also be started from an imported set of people, provided that they can be reasonably integrated. Unintegrated people in a family are undefined behaviour and should be avoided. (Perhaps a verifier could be made in the future?)

```mermaid
graph LR
    p1[Spouse]
    p2[Spouse]
    r[Child]
    u[Unintegrated]

    p1 --- r
    p2 --- r
    r --> ...
    u

    classDef reg fill: #0000, stroke: #fff, color: #fff
    classDef dash fill: #0000, stroke: #fff, stroke-dasharray: 2, color: #fff
    class r,... reg
    class p1,p2,u dash
```

At it's most basic, the model would only track births, deaths, and offspring, using a general probability model to calculate these events. As development goes on, this can be expanded by ways of new events, relations and feats for each person, as well as with family-wide modifiers.

The result of the simulation is then saved into a database containing the data of every person involved.

## Person

A `Person` is a data object which represents a person and their entire life in the context of a `Family`.

A sample person:
```TOML
[[Person]]
id = 0024
name = "Jay Random"
events = [
    { birth = 1900 },
    { marriage = 1930 },
    { death = 1980 },
    # ...
]
relations = [
    { parent = 0009 },
    { parent = 0012 },
    { spouse = 0028 },
    # ...
]
feats = [
    chaotic,
    evil,
    # ...
]
```
* **`id`**&ensp;This field holds a unique identifier which is used as a key for look-ups.
* **`name`**&ensp;Stores a string to be used as a name for the person
* **`events`**&ensp;Collection of events, each with a corresponding date. Events describe the timeline of a person's life. Events should be sorted by date
* **`relations`**&ensp;Collection of relationships, each with an `id` pointing to a person. Relations should be sorted by id
* **`feats`**&ensp;Set of features. A feature describes characteristics that may affect a persons life model, such as `infertile`, `sick` or `cautious`.
  
The creation of a person requires access to:
* An `id` to be used
* A list of viable names or a method to generate one
* Current year to create the `birth` event
* The `id`'s of its parents
* A list of valid birth `feats` to choose from

The simulation of a person's life requires access to:
* Current year to add new `events`
* A list of "available" people to add new `relations`.
* A method to create new people to add new `relations`.
* A list if viable acquired `feats` to choose from.

### root

A root is the initial `Person` in a given family, it is also the person to which all other people in the family are related, wether directly or indirectly.
