
function toggle (ev) {
  let nodes = ev.target.parentNode.getElementsByClassName('collapse')
  for (let node = 0; node < nodes.length; node++) {
    nodes[node].classList.toggle('in')
  }
}

export default {
  bind (el, binding) {
    el.addEventListener('click', toggle)
  },
  unbind (el, binding) {
    el.removeEventListener('click', toggle)
  }
}
