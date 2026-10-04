use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub const BUFFER_SIZE: usize = 64 * 1024;

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------

#[allow(dead_code)]
pub struct MyBufReader {
    // TODO: добавьте необходимые поля для буферизации чтения
    // возможно, вам понадобится что-то вроде:
    // поле для хранения данных, прочитанных из файла, но ещё не отданных пользователю,
    // или позиция внутри этого буфера, откуда отдавать следующий байт.
    // ну и возможно что-то ещё, что поможет вам реализовать read_byte() эффективно
    file: File,
    buffer: [u8; BUFFER_SIZE],
    current_position: usize,
    readed_bytes_size: usize,
    eof: bool,
}

impl MyBufReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let result = File::open(path);
        match result {
            Err(error) => Err(error),
            Ok(file) => Ok(MyBufReader {
                file,
                buffer: [0; BUFFER_SIZE],
                current_position: 0,
                readed_bytes_size: 0,
                eof: false,
            }),
        }
    }

    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        loop {
            if self.eof {
                return Ok(None);
            }
            if self.current_position < self.readed_bytes_size {
                let b = self.buffer[self.current_position];
                self.current_position += 1;
                return Ok(Some(b));
            }

            let result = self.file.read(&mut self.buffer);
            match result {
                Err(error) => return Err(error),
                Ok(size) => match size {
                    0 => {
                        self.eof = true;
                    }
                    _ => {
                        self.current_position = 0;
                        self.readed_bytes_size = size;
                    }
                },
            }
        }
    }
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    // TODO: добавьте необходимые поля для буферизации записи
    // наверное, вам понадобится буфер для хранения данных, которые пользователь уже передал в
    // write_buffered(),
    // но ещё не записал в файл, и, конечно же, еще какие-то поля для работы с файлом
    file: File,
    buffer: [u8; BUFFER_SIZE],
    current_position: usize,
}

impl MyBufWriter {
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let result = File::create(path);
        match result {
            Err(err) => Err(err),
            Ok(file) => Ok(MyBufWriter {
                file,
                buffer: [0; BUFFER_SIZE],
                current_position: 0,
            }),
        }
    }

    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {
        // let x = self.file.write(data);
        // Ok(())
        let mut data = data;
        loop {
            let chunk_size = data.len();
            let capacity_to_write = BUFFER_SIZE - self.current_position;
            if chunk_size <= capacity_to_write {
                self.buffer[self.current_position..self.current_position + chunk_size]
                    .clone_from_slice(data);
                self.current_position += chunk_size;
                return Ok(());
            }

            self.buffer[self.current_position..BUFFER_SIZE]
                .clone_from_slice(&data[..capacity_to_write]);
            let _ = self.file.write_all(&self.buffer);
            self.current_position = 0;
            data = &data[capacity_to_write..];
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        // TODO: эта функция должна записать в файл все данные, которые сейчас лежат во внутреннем
        // буфере, верно? И после этого внутренний буфер должен быть пустым, готовым для новых
        // данных от пользователя.
        // но пока что просто заглушка, чтобы код компилировался, вам нужно реализовать эту функцию
        if self.current_position > 0 {
            let result = self.file.write_all(&self.buffer[..self.current_position]);
            match result {
                Err(err) => return Err(err),
                _ => {}
            }
        }
        let result = self.file.flush();
        self.current_position = 0;
        result
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
