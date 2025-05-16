import config_lang_reader

a = config_lang_reader.read_toml("../test.toml")
b = config_lang_reader.read_yaml("../test.yaml")
c = config_lang_reader.read_json("../test.json")
d = config_lang_reader.read_xml("../test.xml")

e = config_lang_reader.read("../test.toml")

fail = config_lang_reader.read("../test.html")  # will throw unsupported file extension
pass
