
let onList = []

let lastClick = {
  target: null,
  timeStamp: null
}

function toggle (node, className, on) {
  let nodes = node.getElementsByClassName('collapse')
  if (on === undefined || on === null) {
    on = true
    for (let i = 0; i < nodes.length; i++) {
      if (nodes[i].classList.contains('in')) {
        on = false
      }
    }
  }
  if (on) {
    for (let i = 0; i < nodes.length; i++) {
      nodes[i].classList.add('in')
    }
    onList.push({ node: node, className: className })
  } else {
    for (let i = 0; i < nodes.length; i++) {
      nodes[i].classList.remove('in')
    }
    let i = onList.findIndex((el) => el.className === className && el.node === node)
    if (i >= 0) {
      onList.splice(i, 1)
    }
  }
}

function clickElem (ev) {
  lastClick.target = ev.target
  lastClick.timeStamp = ev.timeStamp
  let node = ev.target.parentNode
  toggle(node, 'collapse')
}

function clickWindow (ev) {
  if (ev.target === lastClick.target && ev.timeStamp === lastClick.timeStamp) {
    return
  }
  if (onList.length > 0) {
    var l = onList.pop()
    toggle(l.node, l.className, false)
  }
}

export default {
  bind (el, binding) {
    el.addEventListener('click', clickElem)
    window.addEventListener('click', clickWindow)
  },
  unbind (el, binding) {
    el.removeEventListener('click', clickElem)
    window.removeEventListener('click', clickWindow)
  }
}
