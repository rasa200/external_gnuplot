use crate::errors::SavingError;

pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

/// Missing documentation.
///
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Data<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: Display,
{
    pub(crate) data: I,
    pub(crate) config: crate::configuration::Configuration,
    pub(crate) dim: u8,
}

impl<I> Data<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: Display,
{
    pub fn new(data: I, dim: u8) -> Self {
        let config = crate::configuration::Configuration::default();
        Data { data, config, dim }
    }

    pub fn set_title<S: Display>(mut self, title: S) -> Self {
        self.config.set_title(title.to_string());
        self
    }
    pub fn set_logx<N: Into<f64>>(mut self, logx: N) -> Self {
        self.config.set_logx(logx.into());
        self
    }
    pub fn set_logy<N: Into<f64>>(mut self, logy: N) -> Self {
        self.config.set_logy(logy.into());
        self
    }
}

impl<I> crate::traits::Preexplorable for Data<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: Display,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn save<S: Display>(self, serie: &S) -> Result<(), SavingError> {
        self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "data";
        std::fs::create_dir_all(data_dir)?;

        let data_name = &format!("{}.txt", serie);
        let path = &format!("{}\\{}", data_dir, data_name);

        // Create the data structure for gnuplot

        let mut data_gnuplot = String::new();
        data_gnuplot.push_str("# index value\n");
        for value in self.data.into_iter() {
            for _ in 0..self.dim {
                data_gnuplot.push_str(&format!("{}\t", value));
            }
            data_gnuplot.push_str("\n");
        }

        // Write the data

        std::fs::write(path, data_gnuplot)?;

        Ok(())
    }

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it.
    ///
    /// # Remark
    ///
    /// The plot will be executed asyncroniously and idependently of the Rust program.
    ///
    fn plot<S: Display>(self, serie: &S) -> Result<(), SavingError> {
        match self.dim {
    		1 => {
    			let sequence = crate::sequence::Sequence::from_raw(self.data, self.config);
    			sequence.plot(serie)
    		},
    		2 => {
    			// separate iterators
    			let mut first_filter = vec![true, false].into_iter().cycle();
    			let mut second_filter = vec![false, true].into_iter().cycle();

    			let first_data = self.data.clone().into_iter().filter(move |_| first_filter.next().unwrap());
    			let second_data = self.data.into_iter().filter(move |_| second_filter.next().unwrap());

    			let process = crate::process::Process::from_raw(first_data, second_data, self.config);
    			process.plot(serie)
    		},
    		_ => return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other, "ploting general data: dimension of data is too high to be automatically ploted. Please do it yourself."
                ).into()
            ),
    	}
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<(), SavingError> {
        match self.dim {
    		1 => {
    			let sequence = crate::sequence::Sequence::from_raw(self.data.clone(), self.config.clone());
    			sequence.write_plot_script(serie)
    		},
    		2 => {
    			// separate iterators
    			let mut first_filter = vec![true, false].into_iter().cycle();
    			let mut second_filter = vec![false, true].into_iter().cycle();

    			let first_data = self.data.clone().into_iter().filter(move |_| first_filter.next().unwrap());
    			let second_data = self.data.clone().into_iter().filter(move |_| second_filter.next().unwrap());

    			let process = crate::process::Process::from_raw(first_data, second_data, self.config.clone());
    			process.write_plot_script(serie)
    		},
    		_ => return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other, "ploting general data: dimension of data is too high to be automatically ploted. Please do it yourself."
                ).into()
            ),
    	}
    }
}
