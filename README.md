# Vain Purple Tiger

This program allows you to create a Helium swarm key that resolves to a name of your chosing within the parameters of the word lists employed by the protocol. E.g. "breezy-blue-badger".

**Warning: This program uses a CSPRNG (`OsRng`) for getting randomness and `helium-crypto-rs` for key generation, but no guarantees are made to the security or correctness of generated keys. Use at your own risk.**

### Flags

* network [-n, --network] optional, default = mainnet

Options: main/mainnet | test/testnet

Specify which network to generate the key for.

* cpus [-c, --cpus] optional, default = max num of cores

Integer, 1 to n, where n is the number of logical cores available on your machine, to use for generating keys. By default, this is greedy and will use all available cores.

* output [-o, --output] optional, default = "swarm_key" in current dir

Specify output path to save key.

Example command:

```
vain-purple-tiger -n test --cpus 4 -o /home/user/swarm_key words --color pastel --animal tiger
> magic-pastel-tiger
```

### Subcommands

* words

Choose the name by specifying one or more of the three categories: adjective, color, animal.

Example command:

```
vain-purple-tiger words --color blue --animal badger
> skinny-blue-badger
```

* letter

Choose the name by specifying a letter for each word to begin with in an alliterative fashion.

Example command:

```
vain-purple-tiger letter p
> precise-pastel-porpoise

```

* regex

Specify your own regex for matching. Note, the regex input is not checked for validity against the word lists, so specifying an invalid regex will result in the program running until killed.

Example command:

```
vain-purple-tiger regex gl[a-z]+-white-[a-z]+
> glorious-white-boar"
```

* lists

Print out the word lists.

Example command:

```
vain-purple-tiger lists
> Adjectives: ["attractive", "bald", "beautiful"...]
>
> Colors: ["white", "pearl", "alabaster"...]
>
> Animals: ["alligator", "bee", "bird"...]
```



### Performance

Performance will vary widely with hardware and luck, but a rough idea of expected results is below:

| Subcommand         | Performance                |
| ------------------ | -------------------------- |
| words, one word    | fast, <1 min               |
| words, two words   | fast, several mins         |
| words, three words | slow, mins to tens of mins |
| letter             | fast, <1 min               |
| regex              | variable, based on regex   |



### Credits

* [OrmEmbaar](https://github.com/OrmEmbaar) of Factoshi for the idea, access to their Go implementation, and code reviews.

* [madninja](https://github.com/madninja) and the Helium community for use of, and liberal references to: [angry-purple-tiger](https://github.com/helium/angry-purple-tiger-rs), [helium-crypto-rs](https://github.com/helium/helium-crypto-rs), [helium-wallet-rs](https://github.com/helium/helium-wallet-rs).