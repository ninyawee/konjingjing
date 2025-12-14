import konjingjing


def test_get_id_meaning_valid():
    result = konjingjing.get_id_meaning("1112034563562")
    assert result is not None
    assert result["person_type_code"] == 1
    assert result["person_type_description"] == "คนไทยที่แจ้งเกิดภายในกำหนด"
    assert result["person_type_description_en"] == "Thai citizen, birth registered on time"
    assert result["province_code"] == 11
    assert result["province_name_th"] == "สมุทรปราการ"
    assert result["province_name_en"] == "Samut Prakan"
    assert result["is_valid"] == True


def test_get_id_meaning_bangkok():
    result = konjingjing.get_id_meaning("1101700230703")
    assert result is not None
    assert result["person_type_code"] == 1
    assert result["province_code"] == 10
    assert result["province_name_th"] == "กรุงเทพมหานคร"
    assert result["province_name_en"] == "Bangkok"
    assert result["amphoe_code"] == 1017
    assert result["amphoe_name"] == "ห้วยขวาง"


def test_get_id_meaning_person_type_2():
    # Person type 2: late birth registration
    result = konjingjing.get_id_meaning("2101700230702")
    assert result is not None
    assert result["person_type_code"] == 2
    assert "แจ้งเกิดเกินกำหนด" in result["person_type_description"]
    assert result["person_type_description_en"] == "Thai citizen, birth registered late"


def test_get_id_meaning_invalid_format():
    assert konjingjing.get_id_meaning("") is None
    assert konjingjing.get_id_meaning("123") is None
    assert konjingjing.get_id_meaning("abcdefghijklm") is None


def test_get_id_meaning_invalid_person_type():
    # Person type 0 and 9 are invalid
    assert konjingjing.get_id_meaning("0101700230704") is None
    assert konjingjing.get_id_meaning("9101700230701") is None


def test_get_id_meaning_unknown_province():
    # Province code 99 doesn't exist
    result = konjingjing.get_id_meaning("1991700230700")
    assert result is not None
    assert result["province_code"] is None
    assert result["province_name_th"] is None
