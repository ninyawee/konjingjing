# Person Types

The first digit of a Thai National ID indicates the person type category. There are 8 categories (codes 1-8).

## Type Codes

| Code | Thai Description | English Description |
|------|------------------|---------------------|
| 1 | คนไทยที่แจ้งเกิดภายในกำหนด | Thai citizen, birth registered on time |
| 2 | คนไทยที่แจ้งเกิดเกินกำหนด | Thai citizen, birth registered late |
| 3 | คนไทยหรือต่างด้าวที่มีทะเบียนบ้านก่อน 31 พ.ค. 2527 | Thai or foreigner registered before May 31, 1984 |
| 4 | คนไทยหรือต่างด้าวที่ย้ายเข้าโดยไม่มีเลขประจำตัวในสมัยเริ่มแรก | Thai or foreigner who moved in without ID number at initial period |
| 5 | คนไทยที่เพิ่มชื่อในทะเบียนบ้านกรณีตกสำรวจ | Thai citizen added to house registration (census omission) |
| 6 | ผู้เข้าเมืองโดยไม่ถูกกฎหมายหรืออยู่ชั่วคราว | Illegal immigrant or temporary resident |
| 7 | บุตรของบุคคลประเภท 6 ที่เกิดในไทย | Child of type 6 person, born in Thailand |
| 8 | ต่างด้าวถูกกฎหมายหรือแปลงสัญชาติเป็นไทย | Legal foreigner or naturalized Thai citizen |

## Detailed Descriptions

### Type 1 - Birth Registered on Time

Most common type. Assigned to Thai citizens whose birth was registered within the legal timeframe (15 days from birth).

### Type 2 - Birth Registered Late

Assigned to Thai citizens whose birth was registered after the legal deadline.

### Type 3 - Pre-1984 Registration

Assigned to Thai citizens or foreigners who had house registration before May 31, 1984 (before the current ID system was implemented).

### Type 4 - Initial Period Migration

Assigned to Thai citizens or foreigners who moved into a new house registration without having an ID number during the initial implementation period of the ID system.

### Type 5 - Census Omission

Assigned to Thai citizens who were added to house registration after being discovered as missing from the census.

### Type 6 - Temporary Residents

Assigned to illegal immigrants or temporary residents without permanent status.

### Type 7 - Children of Type 6

Assigned to children born in Thailand to parents who are Type 6 (temporary residents or illegal immigrants).

### Type 8 - Legal Foreigners

Assigned to legally residing foreigners or those who have been naturalized as Thai citizens.

## Usage in Code

```python
from konjingjing import get_id_meaning

meaning = get_id_meaning("1101700230703")
if meaning:
    print(f"Type {meaning['person_type_code']}: {meaning['person_type_description_en']}")
    # Output: Type 1: Thai citizen, birth registered on time
```
