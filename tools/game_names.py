"""Read name slots from an owned MHGU English GMD dump; outputs stay local."""
import struct

def strings(path):
    data=path.read_bytes()
    if len(data)<41 or data[:4]!=b'GMD\0':
        raise ValueError(f'unsupported or truncated GMD: {path.name}')
    start=41+struct.unpack_from('<I',data,36)[0]
    if start>len(data):
        raise ValueError(f'truncated GMD strings: {path.name}')
    return [s.decode('utf-8') for s in data[start:].split(b'\0')][:-1]

def names(path, stride):
    rows=strings(path)
    return {str(i//stride):rows[i] for i in range(0,len(rows),stride) if rows[i]}

def armor_names(path):
    rows=strings(path);result={str(slot):{} for slot in range(5)}
    for slot in range(5):
        matches={}
        for offset in range(slot,len(rows),10):
            if rows[offset]:matches.setdefault(rows[offset],[]).append(offset//10)
        result[str(slot)]={name:ids[0] for name,ids in matches.items() if len(ids)==1}
    return result

def export(table):
    weapons={str(i):names(table/f'weapon{i:02d}MsgData_eng.gmd',6)
             for i in range(15) if i!=5}
    # MHGU 1.4 internal bowgun shots duplicate physical ammo names. Exclude
    # those virtual-only IDs, matching the tested read-only inventory mapping.
    items={key:value for key,value in names(table/'itemData_eng.gmd',2).items()
           if not 132<=int(key)<=204}
    return {'weapons':weapons},items
