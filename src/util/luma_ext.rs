// It often just so happes that people have the screen and luma at the same
// time and need a reference to the link. This method solve that by doing the
// thing and returning a refernce to it as well as its index withing the list.
// It also returns the index of the tab.
// pub fn get_link<'a>(screen: &Screen, luma: &'a Luma) -> Option<(&'a Link, usize)> {
//     let tab = screen.get_selected_tab();
//     let index = screen.get_selected_index();
//     let set = luma.get_set_by_index(tab.into());
//     let link = index.and_then(|i| set.and_then(|s| s.get(i)));
//     link.zip(index)
// }

// The little bro of [`get_link`]. Just queryies the set when you have both
// the screen and the luma and returns the index of where it was found
// pub fn get_set<'a>(screen: &Screen, luma: &'a Luma) -> (&'a [Link], usize) {
//     let tab = screen.get_selected_tab();
//     let set = luma.get_set_by_index(tab.into()).unwrap();
//     (set, tab)
// }

// Mutable version of [`get_set`].
// pub fn get_set_mut<'a>(screen: &Screen, luma: &'a mut Luma) -> (&'a mut [Link], ScreenType) {
//     let tab = screen.get_selected_tab();
//     let set = luma.get_mut_set_by_index(tab.into()).unwrap();
//     (set, tab)
// }
