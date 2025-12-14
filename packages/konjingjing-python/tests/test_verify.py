import konjingjing


def test_valid_id():
    assert konjingjing.verify_id("1112034563562")


def test_invalid_checksum():
    assert not konjingjing.verify_id("1101700230705")


def test_too_short():
    assert not konjingjing.verify_id("110170023073")


def test_contains_letters():
    assert not konjingjing.verify_id("11017002070d3")
    assert not konjingjing.verify_id("rytege54fsfsf")


def test_single_char():
    assert not konjingjing.verify_id("0")
    assert not konjingjing.verify_id("-")


def test_empty():
    assert not konjingjing.verify_id("")


def test_garbage():
    assert not konjingjing.verify_id("blablabla")
