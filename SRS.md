Dynasties is an application that procedurally generates a genealogy.

## Planned features

### Essential

* Procedurally generate a genealogy.
  * Use a simple probabilistic model.
  * Use a year model relative to the root of the family.
* Save genealogy as a TOML file.

### Important

* Load a previously saved TOML genealogy.
* Use custom name-lists in generation.
* Use different probabilistic models.

### Useful

* Navigate family from the program.
* Export file as GEDCOM 5.5 and/or GEDCOM X.
* Re-generate subfamilies on demand.

### Extras

* Use a simulationist model for generation.
  * Implement traits to affect probabilities.
  * Implement flavourful events.
  * Run a month-by-month model.
* Deep customization
  * Custom family models (patrilinearity, matrilinearity, ambilinearity, etc.).
  * Custom traits
  * Custom events
* Graphical interface (text-based or otherwise)

## Version goals

### 0.1.0

* Procedurally generate a genealogy.
  * Use a simple probabilistic model.
  * Use a year model relative to the root of the family.

### 0.2.0

* Save genealogy as a TOML file.

### 0.3.0

* Use custom name-lists in generation.
* Use custom probabilistic models.

### 1.0.0

* Navigate family from within the program.

### 1.1.0

* Edit individuals information.

### 1.2.0

* Regenerate family branches on demand.

### 1.3.0

* Export to alternate file formats (GEDCOM)

### 2.x.x

* Use a simulationist model for generation.
  * Implement traits to affect probabilities.
  * Implement flavourful events.
  * Run a month-by-month model.
* Deep customization
  * Custom family models (patrilinearity, matrilinearity, ambilinearity, etc.).
  * Custom traits
  * Custom events

### 3.x.x

* Graphical interface (text-based or otherwise)

## Implementation

### Definitions

* **Core**: The core is a starting template for a family, it includes a fully fleshed out individual who will serve as a *root* for the family, and their two parents, barebones individuals consisting of only a name and their relation with the *root*
* **Root**: An individual from whom an entire family stems. That is, each child that belongs to a family should be able to tract its lineage back to the root.

### Family

A **family** is a data structure:

* A graph to represent individuals and their relations.
  * Nodes are individuals
    * Id
    * Name
      * Family name
      * Given name
    * Gender
    * Date of birth
    * Date of death
  * Edges are their relations
    * Parent of X
    * Child of X
    * Spouse of X from Y
    * Ex-spouse of X from Y to Z
* Current date

Operations on a family should include:

* **New family**: Initialize a new family with a starting *core*.
* **Simulate year**: Procedurally step through and evaluate all individuals within the family, executing appropriate functions as determined by the probability model and the limitations of each individual.
* **Export family**: Save generated data as a TOML file that includes all pertinent information of each individual of the family.

### Probabilistic model

* **Fertility**: chance of a woman bearing a child as a function of age, partner age, civil status and time since last birth.
* **Mortality**: chance of death as a function of age.
* **Marriage rate**: chance of marriage as a function of age, civil status, and total number of partners.
* **Divorce rate**: chance of divorce as a product of length of marriage, number of children and number of previous divorces among both spouses.
