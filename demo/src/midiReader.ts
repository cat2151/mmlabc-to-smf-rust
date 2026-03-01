// Reads binary data from a Standard MIDI File buffer
export class MidiReader {
    private data: Uint8Array;
    public offset: number = 0;

    constructor(data: Uint8Array) {
        this.data = data;
    }

    get length(): number {
        return this.data.length;
    }

    readUint8(): number {
        if (this.offset >= this.data.length) {
            throw new Error('Unexpected EOF while reading uint8');
        }
        return this.data[this.offset++];
    }

    readUint16(): number {
        if (this.offset + 2 > this.data.length) {
            throw new Error('Unexpected EOF while reading uint16');
        }
        const value = (this.data[this.offset] << 8) | this.data[this.offset + 1];
        this.offset += 2;
        return value & 0xffff;
    }

    readUint32(): number {
        if (this.offset + 4 > this.data.length) {
            throw new Error('Unexpected EOF while reading uint32');
        }
        const value =
            (this.data[this.offset] << 24) |
            (this.data[this.offset + 1] << 16) |
            (this.data[this.offset + 2] << 8) |
            this.data[this.offset + 3];
        this.offset += 4;
        return value >>> 0;
    }

    readVlq(): number {
        let value = 0;
        let bytesRead = 0;
        while (true) {
            if (this.offset >= this.data.length) break;
            const b = this.data[this.offset++];
            value = (value << 7) | (b & 0x7f);
            bytesRead++;
            if (!(b & 0x80) || bytesRead > 4) break;
        }
        return value >>> 0;
    }

    readChar4(): string {
        if (this.offset + 4 > this.data.length) {
            throw new Error('Unexpected EOF while reading char4');
        }
        const result = String.fromCharCode(
            this.data[this.offset],
            this.data[this.offset + 1],
            this.data[this.offset + 2],
            this.data[this.offset + 3]
        );
        this.offset += 4;
        return result;
    }

    skip(n: number): void {
        this.offset += n;
    }
}
