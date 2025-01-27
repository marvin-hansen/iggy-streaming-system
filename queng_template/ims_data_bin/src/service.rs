use trait_data_integration::ImsDataIntegration;

pub(crate) struct ImsDataService<Integration>
where
    Integration: ImsDataIntegration,
{
    pub(crate) intersection: Integration,
}

impl<Integration> ImsDataService<Integration>
where
    Integration: ImsDataIntegration,
{
    pub fn new(intersection: Integration) -> Self {
        Self { intersection }
    }
}
