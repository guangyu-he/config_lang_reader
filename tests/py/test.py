import config_lang_serder

a = config_lang_serder.read_toml("../test.toml")

# write test
config_lang_serder.write_toml(a, "../test_created.toml")

b = config_lang_serder.read_yaml("../test.yaml")
c = config_lang_serder.read_json("../test.json")
d = config_lang_serder.read_xml("../test.xml")

e = config_lang_serder.read("../test.toml")

fail = config_lang_serder.read("../test.html")  # will throw unsupported file extension
pass
