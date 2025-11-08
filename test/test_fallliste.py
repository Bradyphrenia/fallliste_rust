import fallliste_rust

TESTFALL_EINGABE = ['54000691', 'Raschke', 'Mario', '23.08.1982', '12.11.2024 06:00', '12.11.2024 18:00', '1', 'G24M',
                    '9', '0,0', '0,0', '0,0', '0', '0,0', '0,0', '0,0', '0', '0,0', '0,0', '0', '0,0', '0,0', '9999,0',
                    '0', '0,0', '0,0', '0', '0,0', '1', '0,0', '0,0', 'HA Chirurgie',
                    'Allg. Chirurgie/Viszeralchirurgie', 'Allg. Chirurgie/Viszeralchirurgie', 'Hybrid-DRG der DRG G24C',
                    '310512', 'Mario Raschke ', '331094897', '162221', '10031', 'CHIRURGIE', '10032', 'CHI', '10041',
                    'STAT2', '']

ERWARTETE_FELD_ANZAHL = 46


def _anzahl_felder(s: str) -> int:
    return len(s)


def test_eingabe_hat_erwartete_feldanzahl():
    assert _anzahl_felder(TESTFALL_EINGABE) == ERWARTETE_FELD_ANZAHL


def test_sql_part_falliste_gibt_tuple_mit_14_elementen_zurueck():
    res = fallliste_rust.sql_part_falliste(TESTFALL_EINGABE)
    # Erwartet: 14 Felder laut Implementierungsauswahl
    assert isinstance(res, tuple)
    assert len(res) == 14


def test_sql_part_falliste_gibt_tuple_mit_14_elementen_zurueck():
    res = fallliste_rust.sql_part_falliste(TESTFALL_EINGABE)
    assert len(res) == 14


def test_name():
    name = fallliste_rust.sql_part_falliste(TESTFALL_EINGABE)[1]
    assert name == '03e92f4b9b3cdb400638e68ace4cc007'


def test_type():
    assert isinstance(fallliste_rust.sql_part_falliste(TESTFALL_EINGABE), tuple)
