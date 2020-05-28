
let lastClick = {}

function childWithClass (node, className) {
  let nodes = node.children
  for (let i = 0; i < nodes.length; i++) {
    let list = nodes[i].classList
    if (list && list.contains(className)) {
      return nodes[i]
    }
  }
  return null
}

function toggle (ev, node, className, on) {
  let menu = childWithClass(node, 'dropdown-menu')
  if (on === undefined || on === null) {
    on = !menu.classList.contains(className)
  }
  if (on) {
    menu.classList.add(className)
    lastClick = { node: node }
  } else {
    menu.classList.remove(className)
    lastClick = {}
  }
}

function clickElem (ev, node) {
  toggle(ev, node, 'show')
  ev.stopPropagation()
}

function clickWindow (ev) {
  if (!lastClick.node) {
    return
  }
  toggle(ev, lastClick.node, 'show', false)
}

function keyUp (ev) {
  if (lastClick.node && ev.keyCode === 27) {
    toggle(ev, lastClick.node, 'show', false)
  }
}

let click

export default {
  bind (el) {
    click = (ev) => { clickElem(ev, el) }
    el.addEventListener('click', click)
    window.addEventListener('click', clickWindow)
    window.addEventListener('keyup', keyUp)
  },
  unbind (el) {
    el.removeEventListener('click', click)
    window.removeEventListener('click', clickWindow)
    window.removeEventListener('keyup', keyUp)
  }
}
